#include "behaviortree_cpp/bt_factory.h"
#include "./nodes/beacon.cpp"
#include "./nodes/gripper.cpp"

using namespace BT;

static const char* xml_text = R"(
 <root main_tree_to_execute = "MainTree" >
     <BehaviorTree ID="MainTree">
        <Sequence name="root_sequence">
            <BeaconSignal   name="Bacon"/>
        </Sequence>
     </BehaviorTree>
 </root>
 )";

int main()
{
    static GripperInterface gripper;
    BehaviorTreeFactory factory;


    factory.registerNodeType<BeaconSignal>("BeaconSignal");
    factory.registerSimpleAction("Pickup", std::bind(&GripperInterface::pickup, &gripper));
    factory.registerSimpleAction("PutDown", std::bind(&GripperInterface::put_down, &gripper));
    auto tree = factory.createTreeFromText(xml_text);

    tree.tickWhileRunning();

    return 0;
}
