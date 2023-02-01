#include "behaviortree_cpp/behavior_tree.h"
#include <behaviortree_cpp/action_node.h>
#include <behaviortree_cpp/basic_types.h>
#include "../sim/c0.cpp"


using BT::NodeStatus;

C0 *sim;

class SimInterface
{
    int time= 2000000;
  public:
    SimInterface(){}

    NodeStatus retrieve_cap(){
        sim->retrieve();
        return BT::NodeStatus::SUCCESS;

    }
    NodeStatus get_base(){
        sim->getBase();
        return BT::NodeStatus::SUCCESS;
    }

    NodeStatus mount_cap(){
        sim->mount();
        return BT::NodeStatus::SUCCESS;
    }
    NodeStatus deliver(){
        sim->deliver();
        return BT::NodeStatus::SUCCESS;
    }
};


