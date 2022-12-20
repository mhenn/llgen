#include "behaviortree_cpp/bt_factory.h"
#include "./nodes/beacon.cpp"
#include "./nodes/retrieve_cap.cpp"
#include "./nodes/get_base.cpp"
#include "./nodes/mount_cap.cpp"
#include "./nodes/interaction.cpp"
#include "./proto/client.cpp"

using namespace BT;



//         <Parallel>
//            <Repeat num_cycles='-1'>
//                <Beacon name="Bacon"/>
//            </Repeat>

static const char* xml_text = R"(
 <root main_tree_to_execute = "MainTree" >
     <include path="../src/Genetic.xml"/>
     <BehaviorTree ID="MainTree">
            <Sequence name="rootseq">
                <Action ID="PRODUCTION"/>
                <SubTree ID="GP"/>
            </Sequence>
     </BehaviorTree>
 </root>
 )";


static const char* xml_test = R"(
 <root main_tree_to_execute = "MainTree" >
     <BehaviorTree ID="MainTree">
            <Fallback>
                <Repeat num_cycles="-1">
                    <Beacon name="Bacon"/>
                </Repeat>
            </Fallback>
     </BehaviorTree>
 </root>

)";


int main()
{
    static InteractionInterface interaction_if;
    BehaviorTreeFactory factory;

  //  factory.registerNodeType<Beacon>("Beacon");
    //factory.registerSimpleAction("RETRIEVE_CAP", std::bind(&InteractionInterface::retrieve_cap,&interaction_if ));
   // factory.registerSimpleAction("GET_BASE", std::bind(&InteractionInterface::get_base,&interaction_if ));
//    factory.registerSimpleAction("INPUT_BASE", std::bind(&InteractionInterface::base_to_cs,&interaction_if ));
    //factory.registerSimpleAction("MOUNT_CAP", std::bind(&InteractionInterface::mount_cap,&interaction_if ));
    factory.registerSimpleAction("DELIVER", std::bind(&InteractionInterface::deliver,&interaction_if ));
    factory.registerSimpleAction("PRODUCTION", std::bind(&InteractionInterface::production,&interaction_if ));
    factory.registerNodeType<RetrieveCap>("RetrieveCap");
    factory.registerNodeType<GetBase>("GetBase");
    factory.registerNodeType<MountCap>("MountCap");
    auto tree = factory.createTreeFromText(xml_text);
//    auto tree = factory.createTreeFromText(xml_test);
//    auto blackboard = tree.rootBlackboard();
//    blackboard->set("test", false);


    setup_proto();
    setup_client();


    tree.tickWhileRunning();

    delete timer_;
	delete peer_team_;
	delete peer_public_;

	// Delete all global objects allocated by libprotobuf
	google::protobuf::ShutdownProtobufLibrary();

    return 0;
}
