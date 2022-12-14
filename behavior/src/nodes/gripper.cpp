#include "./gripper.h"

BT::NodeStatus GripperInterface::pickup(){
    return BT::NodeStatus::SUCCESS;
}

BT::NodeStatus GripperInterface::put_down(){
    return BT::NodeStatus::SUCCESS;
}
