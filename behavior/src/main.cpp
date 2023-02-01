#include "behaviortree_cpp/bt_factory.h"
#include "./nodes/beacon.cpp"
#include "./nodes/retrieve_cap.cpp"
#include "./nodes/get_base.cpp"
#include "./nodes/mount_cap.cpp"
#include "./nodes/interaction.cpp"
#include "./proto/client.cpp"

#include "./nodes/sim_interaction.cpp"

using namespace BT;



//         <Parallel>
//            <Repeat num_cycles='-1'>
//                <Beacon name="Bacon"/>
//            </Repeat>

static const char* xml_text = R"(
 <root main_tree_to_execute = "MainTree" >
     <include path="../xml/generated.xml"/>
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

static const char* xml_sim = R"(
 <root main_tree_to_execute = "MainTree" >
     <include path="../xml/generated.xml"/>
     <BehaviorTree ID="MainTree">
                <SubTree ID="GP"/>
     </BehaviorTree>
 </root>
 )";



int main()
{
    //static InteractionInterface interaction_if;
    SimInterface interaction_if;
    BehaviorTreeFactory factory;
    sim = new C0();

// WITH REFBOX
//   factory.registerSimpleAction("DELIVER", std::bind(&InteractionInterface::deliver,&interaction_if ));
//    factory.registerSimpleAction("PRODUCTION", std::bind(&InteractionInterface::production,&interaction_if ));
//    factory.registerNodeType<RetrieveCap>("RetrieveCap");
//    factory.registerNodeType<GetBase>("GetBase");
//    factory.registerNodeType<MountCap>("MountCap");
//    auto tree = factory.createTreeFromText(xml_text);

//    setup_proto();
//    setup_client();
///// WITH sim
    factory.registerSimpleAction("RETRIEVE", std::bind(&SimInterface::retrieve_cap,&interaction_if ));
    factory.registerSimpleAction("GETBASE", std::bind(&SimInterface::get_base,&interaction_if ));
    factory.registerSimpleAction("MOUNT", std::bind(&SimInterface::mount_cap,&interaction_if ));
    factory.registerSimpleAction("DELIVER", std::bind(&SimInterface::deliver,&interaction_if ));
    auto tree = factory.createTreeFromText(xml_sim);

/////////
    tree.tickWhileRunning();

    delete timer_;
	delete peer_team_;
	delete peer_public_;

	// Delete all global objects allocated by libprotobuf
	google::protobuf::ShutdownProtobufLibrary();

    return 0;
}
