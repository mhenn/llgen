#include "behaviortree_cpp/behavior_tree.h"
#include <behaviortree_cpp/basic_types.h>

using BT::NodeStatus;

class InteractionInterface
{
  public:
    InteractionInterface()
    {
    }

    NodeStatus retrieve_cap();
    NodeStatus get_base();
    NodeStatus mount_cap();
    NodeStatus deliver();

};

