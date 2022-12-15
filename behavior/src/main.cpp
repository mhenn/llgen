#include "behaviortree_cpp/bt_factory.h"
#include "./nodes/beacon.cpp"
#include "./nodes/gripper.cpp"
#include "./nodes/interaction.cpp"
#include "./proto/client.cpp"

using namespace BT;

static const char* xml_text = R"(
 <root main_tree_to_execute = "MainTree" >

     <include path="../src/Genetic.xml"/>

     <BehaviorTree ID="MainTree">
         <Parallel>
            <Repeat num_cycles="-1">
                <Beacon name="Bacon"/>
            </Repeat>
            <SubTree ID="GP"/>
        </Parallel>
     </BehaviorTree>
 </root>
 )";



int main()
{
    static InteractionInterface inter;
    BehaviorTreeFactory factory;

    factory.registerNodeType<Beacon>("Beacon");
    factory.registerSimpleAction("RETRIEVE_CAP", std::bind(&InteractionInterface::retrieve_cap,&inter ));
    factory.registerSimpleAction("GET_BASE", std::bind(&InteractionInterface::get_base,&inter ));
    factory.registerSimpleAction("INPUT_BASE", std::bind(&InteractionInterface::base_to_cs,&inter ));
    factory.registerSimpleAction("MOUNT_CAP", std::bind(&InteractionInterface::mount_cap,&inter ));
    factory.registerSimpleAction("DELIVER", std::bind(&InteractionInterface::deliver,&inter ));
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
