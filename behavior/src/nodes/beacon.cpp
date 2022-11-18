// Example of custom SyncActionNode (synchronous action)
// without ports.
#include "behaviortree_cpp/behavior_tree.h"
#include "../proto/peer.cpp"


class Beacon : public BT::SyncActionNode
{
public:
  Beacon(const std::string& name) :
      BT::SyncActionNode(name, {})
  {}

  // You must override the virtual function tick()
  BT::NodeStatus tick() override
  {
    handle_beacon();


    return BT::NodeStatus::SUCCESS;
  }
};
