#include <behaviortree_cpp/action_node.h>
#include <behaviortree_cpp/tree_node.h>
#include "../proto/peer.cpp"

class RetrieveCap : public BT::StatefulActionNode
{
  public:
      RetrieveCap(const std::string& name, const BT::NodeConfig& config)
      : BT::StatefulActionNode(name, config)
    {}

    BT::NodeStatus onStart() override
    {
        std::cout << "Retrieve" << std::endl;
        send_action("C-CS1", "CS", "RETRIEVE_CAP");
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
        if(ccs1 == "READY-AT-OUTPUT")
            return BT::NodeStatus::SUCCESS;
        return BT::NodeStatus::RUNNING;
    }

    void onHalted() override
    {
        // nothing to do here...
        std::cout << "SleepNode interrupted" << std::endl;
    }

};
