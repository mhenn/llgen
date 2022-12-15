#include "behaviortree_cpp/behavior_tree.h"
#include <behaviortree_cpp/basic_types.h>
#include "../proto/peer.cpp"

using BT::NodeStatus;

class InteractionInterface
{
  public:
    InteractionInterface(){}

    NodeStatus retrieve_cap(){
        send_action("C-CS1", "CS", "RETRIEVE_CAP");
        usleep(2000000);
        return BT::NodeStatus::SUCCESS;

    }
    NodeStatus get_base(){
        send_action("C-BS", "BS", "", "INPUT", "BASE_RED");
        usleep(2000000);
        return BT::NodeStatus::SUCCESS;
    }
    NodeStatus base_to_cs(){
        send_action("C-BS", "BS", "", "INPUT","BASE_RED");
        usleep(2000000);
        return BT::NodeStatus::SUCCESS;
    }
    NodeStatus mount_cap(){
        send_action("C-CS1", "CS", "MOUNT_CAP");
        usleep(2000000);
        return BT::NodeStatus::SUCCESS;
    }
    NodeStatus deliver(){
        send_action("C-DS", "DS" );
        return BT::NodeStatus::SUCCESS;
    }

};

