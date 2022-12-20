#include <behaviortree_cpp/action_node.h>
#include <behaviortree_cpp/tree_node.h>
#include "../proto/peer.cpp"

class MountCap : public BT::StatefulActionNode
{
    int count = 0;
  public:
      MountCap(const std::string& name, const BT::NodeConfig& config)
      : BT::StatefulActionNode(name, config)
    {}

    BT::NodeStatus onStart() override
    {
        std::cout << "MOUNT" << std::endl;
        send_action("C-CS1", "CS", "MOUNT_CAP");
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
        if(count < 10){
            count++;
            return BT::NodeStatus::RUNNING;
        }

        if( ccs1 == "IDLE"){
            send_action("C-CS1", "CS", "MOUNT_CAP");
            count = 0;
        }
        if( ccs1 == "READY-AT-OUTPUT")
            return BT::NodeStatus::SUCCESS;
        return BT::NodeStatus::RUNNING;
    }

    void onHalted() override
    {
        // nothing to do here...
        std::cout << "SleepNode interrupted" << std::endl;
    }

};
