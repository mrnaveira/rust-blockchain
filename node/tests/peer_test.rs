mod common;

use crate::common::{Api, ServerBuilder};
use serial_test::serial;

#[test]
#[serial]
#[cfg(unix)]
fn test_should_receive_new_valid_blocks() {
    // We will use this node to be the most updated one
    let leader_node = ServerBuilder::new().port(8000).start();

    // This new node will keep asking for new blocks to the leader node
    let mut follower_node = ServerBuilder::new().port(8001).peer(8000).start();

    // At the beggining, both nodes will only have the genesis blocks
    assert_eq!(leader_node.get_blocks().len(), 1);
    assert_eq!(follower_node.get_blocks().len(), 1);

    // we create a new valid block in the leader node
    leader_node.add_valid_block();
    assert_eq!(leader_node.get_blocks().len(), 2);

    // the follower node should eventually ask and add the new block
    follower_node.wait_for_peer_sync();
    assert_eq!(follower_node.get_blocks().len(), 2);

    // the last blocks from both the follower and the leader must match
    let last_leader_block = leader_node.get_last_block();
    let last_follower_block = leader_node.get_last_block();
    assert_eq!(last_follower_block, last_leader_block);
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_not_receive_new_invalid_blocks() {
    // We will use this node to be the most updated one
    let leader_node = ServerBuilder::new().port(8000).start();

    // This new node will keep asking for new blocks to the leader node
    // But we will require a much higher difficulty, so it should not accept blocks from the leader
    let mut follower_node = ServerBuilder::new()
        .difficulty(20)
        .port(8001)
        .peer(8000)
        .start();

    // we create a new valid block in the leader node
    leader_node.add_valid_block();

    // the follower node should eventually ask and receive the new block
    follower_node.wait_for_peer_sync();

    // but the block should not be added as the difficulty will not match
    assert_eq!(follower_node.get_blocks().len(), 1);
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_ignore_unavailable_peers() {
    // We will use this node to be the most updated one
    let leader_node = ServerBuilder::new().port(8000).start();

    // This new node will keep asking for new blocks to the leader node
    // but also to a node that does not exist...
    let mut follower_node = ServerBuilder::new()
        .port(8001)
        .peer(9000)
        .peer(8000)
        .start();

    // we create a new valid block in the leader node
    leader_node.add_valid_block();

    // the follower node should eventually ask and receive the new block
    follower_node.wait_for_peer_sync();

    // even if one of the peers does not exist, it ignores the error and adds blocks from available peers
    assert_eq!(follower_node.get_blocks().len(), 2);
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_send_new_blocks() {
    // This node will always be behind the leader node
    let mut follower_node = ServerBuilder::new().port(8000).start();

    // We will use this node to be the most updated one
    let leader_node = ServerBuilder::new().port(8001).peer(8000).start();

    // At the beggining, both nodes will only have the genesis blocks
    assert_eq!(leader_node.get_blocks().len(), 1);
    assert_eq!(follower_node.get_blocks().len(), 1);

    // we create a new valid block in the leader node
    leader_node.add_valid_block();
    assert_eq!(leader_node.get_blocks().len(), 2);

    // the follower node should eventually receive and add the new block
    follower_node.wait_to_receive_block_in_api();
    assert_eq!(follower_node.get_blocks().len(), 2);

    // the last blocks from both the follower and the leader must match
    let last_leader_block = leader_node.get_last_block();
    let last_follower_block = leader_node.get_last_block();
    assert_eq!(last_follower_block, last_leader_block);
}
