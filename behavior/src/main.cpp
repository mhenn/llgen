#include "behaviortree_cpp/bt_factory.h"
#include "./nodes/beacon.cpp"
#include "./nodes/gripper.cpp"
#include "./proto/client.cpp"

using namespace BT;

static const char* xml_text = R"(
 <root main_tree_to_execute = "MainTree" >
     <BehaviorTree ID="MainTree">
            <Repeat num_cycles="-1">
                <Beacon name="Bacon"/>
            </Repeat>
     </BehaviorTree>
 </root>
 )";


int main()
{
    static GripperInterface gripper;
    BehaviorTreeFactory factory;

    factory.registerNodeType<Beacon>("Beacon");
    factory.registerSimpleAction("Pickup", std::bind(&GripperInterface::pickup, &gripper));
    factory.registerSimpleAction("PutDown", std::bind(&GripperInterface::put_down, &gripper));
    auto tree = factory.createTreeFromText(xml_text);

    setup_proto();
    setup_client();


    tree.tickWhileRunning();

	delete peer_team_;
	delete peer_public_;

	// Delete all global objects allocated by libprotobuf
	google::protobuf::ShutdownProtobufLibrary();

    return 0;
}
