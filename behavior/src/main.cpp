#include "behaviortree_cpp/bt_factory.h"

using namespace BT;

static const char* xml_text = R"(
 <root main_tree_to_execute = "MainTree" >
     <BehaviorTree ID="MainTree">
        <Sequence name="root_sequence">
            <CloseGripper   name="close_gripper"/>
        </Sequence>
     </BehaviorTree>
 </root>
 )";

int main()
{
  BehaviorTreeFactory factory;

  auto tree = factory.createTreeFromText(xml_text);

  tree.tickWhileRunning();

  return 0;
}
