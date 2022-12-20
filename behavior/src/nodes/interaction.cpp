#include "behaviortree_cpp/behavior_tree.h"
#include <behaviortree_cpp/action_node.h>
#include <behaviortree_cpp/basic_types.h>
#include "../proto/peer.cpp"

using BT::NodeStatus;

class InteractionInterface
{
    int time= 2000000;
  public:
    InteractionInterface(){}

    NodeStatus retrieve_cap(){
        std::cout << "RETRIEVE" << std::endl;
        send_action("C-CS1", "CS", "RETRIEVE_CAP");
        usleep(time);
        return BT::NodeStatus::SUCCESS;

    }
    NodeStatus get_base(){
        std::cout << "BASE" << std::endl;
        send_action("C-BS", "BS", "", "INPUT", "BASE_RED");
            return BT::NodeStatus::SUCCESS;
    }
//    NodeStatus base_to_cs(){
//        send_action("C-BS", "BS", "", "INPUT","BASE_RED");
//        usleep(time);
//        return BT::NodeStatus::SUCCESS;
//    }

//    NodeStatus mount_cap(){
//        usleep(time);
//        usleep(time);
//        std::cout << "MOUNT" << std::endl;
//        send_action("C-CS1", "CS", "MOUNT_CAP");
//    }
    NodeStatus deliver(){
        usleep(time);
        std::cout << "DELIVER" << std::endl;
        send_action("C-DS", "DS" );
        usleep(time);
        return BT::NodeStatus::SUCCESS;
    }

    NodeStatus production(){
        usleep(time);
        usleep(time);
        return BT::NodeStatus::SUCCESS;
    }

};


