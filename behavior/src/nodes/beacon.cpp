// Example of custom SyncActionNode (synchronous action)
// without ports.
#include "behaviortree_cpp/behavior_tree.h"

class Beacon : public BT::SyncActionNode
{
public:
  Beacon(const std::string& name) :
      BT::SyncActionNode(name, {})
  {}

  // You must override the virtual function tick()
  BT::NodeStatus tick() override
  {
    std::cout << "BeaconSignal: " << this->name() << std::endl;
    return BT::NodeStatus::SUCCESS;
  }
};
