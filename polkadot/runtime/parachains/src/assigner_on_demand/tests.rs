// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use super::*;

use crate::{
	assigner_on_demand::{mock_helpers::GenesisConfigBuilder, Error},
	initializer::SessionChangeNotification,
	mock::{
		new_test_ext, Balances, OnDemandAssigner, Paras, ParasShared, RuntimeOrigin, Scheduler,
		System, Test,
	},
	paras::{ParaGenesisArgs, ParaKind},
};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use pallet_balances::Error as BalancesError;
use primitives::{BlockNumber, SessionIndex, ValidationCode};
use sp_std::collections::btree_map::BTreeMap;

fn schedule_blank_para(id: ParaId, parakind: ParaKind) {
	let validation_code: ValidationCode = vec![1, 2, 3].into();
	assert_ok!(Paras::schedule_para_initialize(
		id,
		ParaGenesisArgs {
			genesis_head: Vec::new().into(),
			validation_code: validation_code.clone(),
			para_kind: parakind,
		}
	));

	assert_ok!(Paras::add_trusted_validation_code(RuntimeOrigin::root(), validation_code));
}

fn run_to_block(
	to: BlockNumber,
	new_session: impl Fn(BlockNumber) -> Option<SessionChangeNotification<BlockNumber>>,
) {
	while System::block_number() < to {
		let b = System::block_number();

		Scheduler::initializer_finalize();
		Paras::initializer_finalize(b);

		if let Some(notification) = new_session(b + 1) {
			let mut notification_with_session_index = notification;
			// We will make every session change trigger an action queue. Normally this may require
			// 2 or more session changes.
			if notification_with_session_index.session_index == SessionIndex::default() {
				notification_with_session_index.session_index = ParasShared::scheduled_session();
			}
			Paras::initializer_on_new_session(&notification_with_session_index);
			Scheduler::initializer_on_new_session(&notification_with_session_index);
		}

		System::on_finalize(b);

		System::on_initialize(b + 1);
		System::set_block_number(b + 1);

		Paras::initializer_initialize(b + 1);
		Scheduler::initializer_initialize(b + 1);

		// In the real runtime this is expected to be called by the `InclusionInherent` pallet.
		Scheduler::free_cores_and_fill_claimqueue(BTreeMap::new(), b + 1);
	}
}

#[test]
fn spot_traffic_capacity_zero_returns_none() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		0u32,
		u32::MAX,
		Perbill::from_percent(100),
		Perbill::from_percent(1),
	) {
		Ok(_) => panic!("Error"),
		Err(e) => assert_eq!(e, SpotTrafficCalculationErr::QueueCapacityIsZero),
	};
}

#[test]
fn spot_traffic_queue_size_larger_than_capacity_returns_none() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		1u32,
		2u32,
		Perbill::from_percent(100),
		Perbill::from_percent(1),
	) {
		Ok(_) => panic!("Error"),
		Err(e) => assert_eq!(e, SpotTrafficCalculationErr::QueueSizeLargerThanCapacity),
	}
}

#[test]
fn spot_traffic_calculation_identity() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from_u32(1),
		1000,
		100,
		Perbill::from_percent(10),
		Perbill::from_percent(3),
	) {
		Ok(res) => {
			assert_eq!(res, FixedU128::from_u32(1))
		},
		_ => (),
	}
}

#[test]
fn spot_traffic_calculation_u32_max() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from_u32(1),
		u32::MAX,
		u32::MAX,
		Perbill::from_percent(100),
		Perbill::from_percent(3),
	) {
		Ok(res) => {
			assert_eq!(res, FixedU128::from_u32(1))
		},
		_ => panic!("Error"),
	};
}

#[test]
fn spot_traffic_calculation_u32_traffic_max() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		u32::MAX,
		u32::MAX,
		Perbill::from_percent(1),
		Perbill::from_percent(1),
	) {
		Ok(res) => assert_eq!(res, FixedU128::from(u128::MAX)),
		_ => panic!("Error"),
	};
}

#[test]
fn sustained_target_increases_spot_traffic() {
	let mut traffic = FixedU128::from_u32(1u32);
	for _ in 0..50 {
		traffic = OnDemandAssigner::calculate_spot_traffic(
			traffic,
			100,
			12,
			Perbill::from_percent(10),
			Perbill::from_percent(100),
		)
		.unwrap()
	}
	assert_eq!(traffic, FixedU128::from_inner(2_718_103_312_071_174_015u128))
}

#[test]
fn spot_traffic_can_decrease() {
	let traffic = FixedU128::from_u32(100u32);
	match OnDemandAssigner::calculate_spot_traffic(
		traffic,
		100u32,
		0u32,
		Perbill::from_percent(100),
		Perbill::from_percent(100),
	) {
		Ok(new_traffic) =>
			assert_eq!(new_traffic, FixedU128::from_inner(50_000_000_000_000_000_000u128)),
		_ => panic!("Error"),
	}
}

#[test]
fn spot_traffic_decreases_over_time() {
	let mut traffic = FixedU128::from_u32(100u32);
	for _ in 0..5 {
		traffic = OnDemandAssigner::calculate_spot_traffic(
			traffic,
			100u32,
			0u32,
			Perbill::from_percent(100),
			Perbill::from_percent(100),
		)
		.unwrap();
		println!("{traffic}");
	}
	assert_eq!(traffic, FixedU128::from_inner(3_125_000_000_000_000_000u128))
}

#[test]
fn place_order_works() {
	let alice = 1u64;
	let amt = 10_000_000u128;
	let para_id = ParaId::from(111);

	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		// Initialize the parathread and wait for it to be ready.
		schedule_blank_para(para_id, ParaKind::Parathread);

		assert!(!Paras::is_parathread(para_id));

		run_to_block(100, |n| if n == 100 { Some(Default::default()) } else { None });

		assert!(Paras::is_parathread(para_id));

		// Does not work unsigned
		assert_noop!(
			OnDemandAssigner::place_order_allow_death(RuntimeOrigin::none(), amt, para_id),
			BadOrigin
		);

		// Does not work with max_amount lower than fee
		let low_max_amt = 1u128;
		assert_noop!(
			OnDemandAssigner::place_order_allow_death(
				RuntimeOrigin::signed(alice),
				low_max_amt,
				para_id,
			),
			Error::<Test>::SpotPriceHigherThanMaxAmount,
		);

		// Does not work with insufficient balance
		assert_noop!(
			OnDemandAssigner::place_order_allow_death(RuntimeOrigin::signed(alice), amt, para_id),
			BalancesError::<Test, _>::InsufficientBalance
		);

		// Works
		Balances::make_free_balance_be(&alice, amt);
		run_to_block(101, |n| if n == 101 { Some(Default::default()) } else { None });
		assert_ok!(OnDemandAssigner::place_order_allow_death(
			RuntimeOrigin::signed(alice),
			amt,
			para_id
		));
	});
}

#[test]
fn place_order_keep_alive_keeps_alive() {
	let alice = 1u64;
	let amt = 1u128; // The same as crate::mock's EXISTENTIAL_DEPOSIT
	let max_amt = 10_000_000u128;
	let para_id = ParaId::from(111);

	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		// Initialize the parathread and wait for it to be ready.
		schedule_blank_para(para_id, ParaKind::Parathread);
		Balances::make_free_balance_be(&alice, amt);

		assert!(!Paras::is_parathread(para_id));
		run_to_block(100, |n| if n == 100 { Some(Default::default()) } else { None });
		assert!(Paras::is_parathread(para_id));

		assert_noop!(
			OnDemandAssigner::place_order_keep_alive(
				RuntimeOrigin::signed(alice),
				max_amt,
				para_id
			),
			BalancesError::<Test, _>::InsufficientBalance
		);
	});
}

#[test]
fn add_on_demand_order_works() {
	let para_a = ParaId::from(111);
	let order = EnqueuedOrder::new(para_a);

	let mut genesis = GenesisConfigBuilder::default();
	genesis.on_demand_max_queue_size = 1;
	new_test_ext(genesis.build()).execute_with(|| {
		// Initialize the parathread and wait for it to be ready.
		schedule_blank_para(para_a, ParaKind::Parathread);

		// `para_a` is not onboarded as a parathread yet.
		assert_noop!(
			OnDemandAssigner::add_on_demand_order(order.clone(), QueuePushDirection::Back),
			Error::<Test>::InvalidParaId
		);

		assert!(!Paras::is_parathread(para_a));
		run_to_block(100, |n| if n == 100 { Some(Default::default()) } else { None });
		assert!(Paras::is_parathread(para_a));

		// `para_a` is now onboarded as a valid parathread.
		assert_ok!(OnDemandAssigner::add_on_demand_order(order.clone(), QueuePushDirection::Back));

		// Max queue size is 1, queue should be full.
		assert_noop!(
			OnDemandAssigner::add_on_demand_order(order, QueuePushDirection::Back),
			Error::<Test>::QueueFull
		);
	});
}

#[test]
fn spotqueue_push_directions() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		let para_b = ParaId::from(222);
		let para_c = ParaId::from(333);

		schedule_blank_para(para_a, ParaKind::Parathread);
		schedule_blank_para(para_b, ParaKind::Parathread);
		schedule_blank_para(para_c, ParaKind::Parathread);

		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		let order_a = EnqueuedOrder::new(para_a);
		let order_b = EnqueuedOrder::new(para_b);
		let order_c = EnqueuedOrder::new(para_c);

		assert_ok!(OnDemandAssigner::add_on_demand_order(
			order_a.clone(),
			QueuePushDirection::Front
		));
		assert_ok!(OnDemandAssigner::add_on_demand_order(
			order_b.clone(),
			QueuePushDirection::Front
		));

		assert_ok!(OnDemandAssigner::add_on_demand_order(
			order_c.clone(),
			QueuePushDirection::Back
		));

		assert_eq!(OnDemandAssigner::queue_size(), 3);
		assert_eq!(OnDemandAssigner::get_queue(), VecDeque::from(vec![order_b, order_a, order_c]))
	});
}

#[test]
fn pop_assignment_for_core_works() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		let para_b = ParaId::from(110);
		schedule_blank_para(para_a, ParaKind::Parathread);
		schedule_blank_para(para_b, ParaKind::Parathread);

		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		let order_a = EnqueuedOrder::new(para_a);
		let order_b = EnqueuedOrder::new(para_b);
		let assignment_a = Assignment::Pool { para_id: para_a, core_index: CoreIndex(0) };
		let assignment_b = Assignment::Pool { para_id: para_b, core_index: CoreIndex(1) };

		// Pop should return none with empty queue
		assert_eq!(OnDemandAssigner::pop_assignment_for_core(CoreIndex(0)), None);

		// Add enough assignments to the order queue.
		for _ in 0..2 {
			OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
				.expect("Invalid paraid or queue full");

			OnDemandAssigner::add_on_demand_order(order_b.clone(), QueuePushDirection::Back)
				.expect("Invalid paraid or queue full");
		}

		// Queue should contain orders a, b, a, b
		{
			let queue: Vec<EnqueuedOrder> = OnDemandQueue::<Test>::get().into_iter().collect();
			assert_eq!(
				queue,
				vec![order_a.clone(), order_b.clone(), order_a.clone(), order_b.clone()]
			);
		}

		// Popped assignments should be for the correct paras and cores
		assert_eq!(
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0)),
			Some(assignment_a.clone())
		);
		assert_eq!(
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(1)),
			Some(assignment_b.clone())
		);
		assert_eq!(
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0)),
			Some(assignment_a.clone())
		);

		// Queue should contain one left over order
		{
			let queue: Vec<EnqueuedOrder> = OnDemandQueue::<Test>::get().into_iter().collect();
			assert_eq!(queue, vec![order_b.clone(),]);
		}
	});
}

#[test]
fn push_back_assignment_works() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		let para_b = ParaId::from(110);
		schedule_blank_para(para_a, ParaKind::Parathread);
		schedule_blank_para(para_b, ParaKind::Parathread);

		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		let order_a = EnqueuedOrder::new(para_a);
		let order_b = EnqueuedOrder::new(para_b);

		// Add enough assignments to the order queue.
		OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_on_demand_order(order_b.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		// Pop order a
		OnDemandAssigner::pop_assignment_for_core(CoreIndex(0));

		// Para a should have affinity for core 0
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 1);
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().core_idx, CoreIndex(0));

		// Queue should still contain order b
		{
			let queue: Vec<EnqueuedOrder> = OnDemandQueue::<Test>::get().into_iter().collect();
			assert_eq!(queue, vec![order_b.clone()]);
		}

		// Push back order a
		OnDemandAssigner::push_back_assignment(para_a, CoreIndex(0));

		// Para a should have no affinity
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).is_none(), true);

		// Queue should contain orders a, b. A in front of b.
		{
			let queue: Vec<EnqueuedOrder> = OnDemandQueue::<Test>::get().into_iter().collect();
			assert_eq!(queue, vec![order_a.clone(), order_b.clone()]);
		}
	});
}

#[test]
fn affinity_changes_work() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		let core_index = CoreIndex(0);
		schedule_blank_para(para_a, ParaKind::Parathread);

		let order_a = EnqueuedOrder::new(para_a);
		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		// There should be no affinity before starting.
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());

		// Add enough assignments to the order queue.
		for _ in 0..10 {
			OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Front)
				.expect("Invalid paraid or queue full");
		}

		// There should be no affinity before the scheduler pops.
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());

		OnDemandAssigner::pop_assignment_for_core(core_index);

		// Affinity count is 1 after popping.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 1);

		OnDemandAssigner::report_processed(para_a, 0.into());
		OnDemandAssigner::pop_assignment_for_core(core_index);

		// Affinity count is 1 after popping with a previous para.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 1);
		assert_eq!(OnDemandAssigner::queue_size(), 8);

		for _ in 0..3 {
			OnDemandAssigner::pop_assignment_for_core(core_index);
		}

		// Affinity count is 4 after popping 3 times without a previous para.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 4);
		assert_eq!(OnDemandAssigner::queue_size(), 5);

		for _ in 0..5 {
			OnDemandAssigner::report_processed(para_a, 0.into());
			OnDemandAssigner::pop_assignment_for_core(core_index);
		}

		// Affinity count should still be 4 but queue should be empty.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 4);
		assert_eq!(OnDemandAssigner::queue_size(), 0);

		// Pop 4 times and get to exactly 0 (None) affinity.
		for _ in 0..4 {
			OnDemandAssigner::report_processed(para_a, 0.into());
			OnDemandAssigner::pop_assignment_for_core(core_index);
		}
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());

		// Decreasing affinity beyond 0 should still be None.
		OnDemandAssigner::report_processed(para_a, 0.into());
		OnDemandAssigner::pop_assignment_for_core(core_index);
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());
	});
}

#[test]
fn affinity_prohibits_parallel_scheduling() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		let para_b = ParaId::from(222);

		schedule_blank_para(para_a, ParaKind::Parathread);
		schedule_blank_para(para_b, ParaKind::Parathread);

		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		let order_a = EnqueuedOrder::new(para_a);
		let order_b = EnqueuedOrder::new(para_b);

		// There should be no affinity before starting.
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());
		assert!(OnDemandAssigner::get_affinity_map(para_b).is_none());

		// Add 2 assignments for para_a for every para_b.
		OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_on_demand_order(order_b.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		assert_eq!(OnDemandAssigner::queue_size(), 3);

		// Approximate having 1 core.
		for _ in 0..3 {
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0));
		}

		// Affinity on one core is meaningless.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 2);
		assert_eq!(OnDemandAssigner::get_affinity_map(para_b).unwrap().count, 1);
		assert_eq!(
			OnDemandAssigner::get_affinity_map(para_a).unwrap().core_idx,
			OnDemandAssigner::get_affinity_map(para_b).unwrap().core_idx
		);

		// Clear affinity
		OnDemandAssigner::report_processed(para_a, 0.into());
		OnDemandAssigner::report_processed(para_a, 0.into());
		OnDemandAssigner::report_processed(para_b, 0.into());

		// Add 2 assignments for para_a for every para_b.
		OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");
		OnDemandAssigner::report_processed(assignment_b_core_0.clone());

		// Add 2 assignments for para_a for every para_b.
		OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_on_demand_order(order_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_on_demand_order(order_b.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		// Approximate having 3 cores. CoreIndex 2 should be unable to obtain an assignment
		for _ in 0..3 {
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0));
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(1));
			assert_eq!(None, OnDemandAssigner::pop_assignment_for_core(CoreIndex(2)));
		}

		// Affinity should be the same as before, but on different cores.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 2);
		assert_eq!(OnDemandAssigner::get_affinity_map(para_b).unwrap().count, 1);
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().core_idx, CoreIndex(0));
		assert_eq!(OnDemandAssigner::get_affinity_map(para_b).unwrap().core_idx, CoreIndex(1));

		// Clear affinity
		OnDemandAssigner::report_processed(assignment_a.clone());
		OnDemandAssigner::report_processed(assignment_a.clone());
		OnDemandAssigner::report_processed(assignment_b_core_1.clone());

		// There should be no affinity after clearing.
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());
		assert!(OnDemandAssigner::get_affinity_map(para_b).is_none());
	});
}

#[test]
fn on_demand_orders_cannot_be_popped_if_lifecycle_changes() {
	let para_id = ParaId::from(10);
	let core_index = CoreIndex(0);
	let order = EnqueuedOrder::new(para_id);
	let assignment = OnDemandAssignment::new(para_id, core_index);

	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		// Register the para_id as a parathread
		schedule_blank_para(para_id, ParaKind::Parathread);

		assert!(!Paras::is_parathread(para_id));
		run_to_block(10, |n| if n == 10 { Some(Default::default()) } else { None });
		assert!(Paras::is_parathread(para_id));

		// Add two assignments for a para_id with a valid lifecycle.
		assert_ok!(OnDemandAssigner::add_on_demand_order(order.clone(), QueuePushDirection::Back));
		assert_ok!(OnDemandAssigner::add_on_demand_order(order.clone(), QueuePushDirection::Back));

		// First pop is fine
		assert_eq!(OnDemandAssigner::pop_assignment_for_core(core_index), Some(assignment.clone()));

		// Deregister para
		assert_ok!(Paras::schedule_para_cleanup(para_id));

		// Run to new session and verify that para_id is no longer a valid parathread.
		assert!(Paras::is_parathread(para_id));
		run_to_block(20, |n| if n == 20 { Some(Default::default()) } else { None });
		assert!(!Paras::is_parathread(para_id));

		// Second pop should be None.
		OnDemandAssigner::report_processed(assignment.clone());
		assert_eq!(OnDemandAssigner::pop_assignment_for_core(core_index), None);
	});
}
