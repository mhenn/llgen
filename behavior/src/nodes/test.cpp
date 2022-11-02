// Example of custom SyncActionNode (synchronous action)
// without ports.
#include "behaviortree_cpp/behavior_tree.h"

class Test : public BT::SyncActionNode
{
public:
  Test(const std::string& name) :
      BT::SyncActionNode(name, {})
  {}

  // You must override the virtual function tick()
  BT::NodeStatus tick() override
  {
    std::cout << "Test: " << this->name() << std::endl;
    return BT::NodeStatus::SUCCESS;
  }
};
