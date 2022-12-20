#include <behaviortree_cpp/action_node.h>
#include <behaviortree_cpp/tree_node.h>
#include "../proto/peer.cpp"

class GetBase : public BT::StatefulActionNode
{
  public:
      GetBase(const std::string& name, const BT::NodeConfig& config)
      : BT::StatefulActionNode(name, config)
    {}

    BT::NodeStatus onStart() override
    {
        std::cout << "BASE" << std::endl;
        send_action("C-BS", "BS", "", "INPUT", "BASE_RED");
        return BT::NodeStatus::RUNNING;

    }

static BT::PortsList providedPorts()
    {
      // amount of milliseconds that we want to sleep
      return{  };
    }

    /// method invoked by an action in the RUNNING state.
    BT::NodeStatus onRunning() override
    {
        if(cbs == "READY-AT-OUTPUT")
            return BT::NodeStatus::SUCCESS;
        return BT::NodeStatus::RUNNING;
    }

    void onHalted() override
    {
        // nothing to do here...
        std::cout << "SleepNode interrupted" << std::endl;
    }

};
