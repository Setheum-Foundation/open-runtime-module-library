//! Unit tests for the rewards module.

#![cfg(test)]

use super::*;
use mock::*;

#[test]
fn add_share_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 0,
				total_rewards: 0,
				total_withdrawn_rewards: 0,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (0, 0));

		RewardsModule::add_share(&ALICE, &DNAR_POOL, 0);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 0,
				total_rewards: 0,
				total_withdrawn_rewards: 0,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (0, 0));

		RewardsModule::add_share(&ALICE, &DNAR_POOL, 100);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 100,
				total_rewards: 0,
				total_withdrawn_rewards: 0,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (100, 0));

		Pools::<Runtime>::mutate(DNAR_POOL, |pool_info| {
			pool_info.total_rewards += 5000;
			pool_info.total_withdrawn_rewards += 2000;
		});
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 100,
				total_rewards: 5000,
				total_withdrawn_rewards: 2000,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, BOB), (0, 0));

		RewardsModule::add_share(&BOB, &DNAR_POOL, 50);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 150,
				total_rewards: 7500,
				total_withdrawn_rewards: 4500,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, BOB), (50, 2500));

		RewardsModule::add_share(&ALICE, &DNAR_POOL, 150);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 300,
				total_rewards: 15000,
				total_withdrawn_rewards: 12000,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (250, 7500));
	});
}

#[test]
fn claim_rewards_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		RewardsModule::add_share(&ALICE, &DNAR_POOL, 100);
		RewardsModule::add_share(&BOB, &DNAR_POOL, 100);
		Pools::<Runtime>::mutate(DNAR_POOL, |pool_info| {
			pool_info.total_rewards += 5000;
		});
		RewardsModule::add_share(&CAROL, &DNAR_POOL, 200);

		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 400,
				total_rewards: 10000,
				total_withdrawn_rewards: 5000,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (100, 0));
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, BOB), (100, 0));
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, CAROL), (200, 5000));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, ALICE)).unwrap_or(&0)),
			0
		);
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, BOB)).unwrap_or(&0)),
			0
		);
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, CAROL)).unwrap_or(&0)),
			0
		);

		RewardsModule::claim_rewards(&ALICE, &DNAR_POOL);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 400,
				total_rewards: 10000,
				total_withdrawn_rewards: 7500,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (100, 2500));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, ALICE)).unwrap_or(&0)),
			2500
		);

		RewardsModule::claim_rewards(&CAROL, &DNAR_POOL);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 400,
				total_rewards: 10000,
				total_withdrawn_rewards: 7500,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, CAROL), (200, 5000));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, CAROL)).unwrap_or(&0)),
			0
		);

		RewardsModule::claim_rewards(&BOB, &DNAR_POOL);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 400,
				total_rewards: 10000,
				total_withdrawn_rewards: 10000,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, BOB), (100, 2500));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, BOB)).unwrap_or(&0)),
			2500
		);
	});
}

#[test]
fn remove_share_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		RewardsModule::add_share(&ALICE, &DNAR_POOL, 100);
		RewardsModule::add_share(&BOB, &DNAR_POOL, 100);
		Pools::<Runtime>::mutate(DNAR_POOL, |pool_info| {
			pool_info.total_rewards += 10000;
		});

		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 200,
				total_rewards: 10000,
				total_withdrawn_rewards: 0,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (100, 0));
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, BOB), (100, 0));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, ALICE)).unwrap_or(&0)),
			0
		);
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, BOB)).unwrap_or(&0)),
			0
		);

		// remove amount is zero, do not claim interest
		RewardsModule::remove_share(&ALICE, &DNAR_POOL, 0);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 200,
				total_rewards: 10000,
				total_withdrawn_rewards: 0,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (100, 0));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, ALICE)).unwrap_or(&0)),
			0
		);

		RewardsModule::remove_share(&BOB, &DNAR_POOL, 50);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 150,
				total_rewards: 7500,
				total_withdrawn_rewards: 2500,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, BOB), (50, 2500));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, BOB)).unwrap_or(&0)),
			5000
		);

		RewardsModule::remove_share(&ALICE, &DNAR_POOL, 101);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 50,
				total_rewards: 2501,
				total_withdrawn_rewards: 2500,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (0, 0));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, ALICE)).unwrap_or(&0)),
			4999
		);
	});
}

#[test]
fn set_share_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 0,
				total_rewards: 0,
				total_withdrawn_rewards: 0,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (0, 0));

		RewardsModule::set_share(&ALICE, &DNAR_POOL, 100);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 100,
				total_rewards: 0,
				total_withdrawn_rewards: 0,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (100, 0));

		Pools::<Runtime>::mutate(DNAR_POOL, |pool_info| {
			pool_info.total_rewards += 10000;
		});
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 100,
				total_rewards: 10000,
				total_withdrawn_rewards: 0,
			}
		);

		RewardsModule::set_share(&ALICE, &DNAR_POOL, 500);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 500,
				total_rewards: 50000,
				total_withdrawn_rewards: 40000,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (500, 40000));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, ALICE)).unwrap_or(&0)),
			0
		);

		RewardsModule::set_share(&ALICE, &DNAR_POOL, 100);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 100,
				total_rewards: 10000,
				total_withdrawn_rewards: 10000,
			}
		);
		assert_eq!(RewardsModule::share_and_withdrawn_reward(DNAR_POOL, ALICE), (100, 10000));
		assert_eq!(
			RECEIVED_PAYOUT.with(|v| *v.borrow().get(&(DNAR_POOL, ALICE)).unwrap_or(&0)),
			10000
		);
	});
}

#[test]
fn accumulate_reward_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 0,
				total_rewards: 0,
				total_withdrawn_rewards: 0,
			}
		);

		RewardsModule::accumulate_reward(&DNAR_POOL, 100);
		assert_eq!(
			RewardsModule::pools(DNAR_POOL),
			PoolInfo {
				total_shares: 0,
				total_rewards: 100,
				total_withdrawn_rewards: 0,
			}
		);
	});
}
