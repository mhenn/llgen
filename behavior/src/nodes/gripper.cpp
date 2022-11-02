
#include "behaviortree_cpp/behavior_tree.h"
#include <behaviortree_cpp/basic_types.h>

using BT::NodeStatus;

class GripperInterface
{
  public:
    GripperInterface() : _opened(true)
    {
    }

    NodeStatus pickup();

    NodeStatus put_down();

  private:
    bool _opened;
};

