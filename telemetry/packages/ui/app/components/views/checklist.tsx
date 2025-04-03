import React, { useState } from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

const markdown = `
# Procedures List
1. [ ] Ensure all non-essential personnel are cleared from the area to the designated observation zone. 
2. [ ] Transport the pod to the track using the Pod Transport Procedure (see SSOW).
3. [ ] Position the pod on the ground/cart behind the first I-beam segment of the track (shell off).
4. [ ] Pressurise the pneumatic tank through the schrader valve. Use the air compressor to take note of the pressure.
5. [ ] Turn on the three-phase power at the wall, then turn on the power supply at 120V, 50A. Set up power supply with correct voltage and current limits - with the Overvoltage trip set to 140V and overcurrent trip set to 50A, referring to the online manual.
https://magna-power.com/assets/docs/html_ts/
Turn on the master switch, which will activate low power systems (the Master Control Unit) and the pod state machine will enter the IDLE state once all systems initialised (automatically).
6. [ ] Establish wireless communication with the “base station” computer.
7. [ ] From the GUI on the base station, verify all pod systems are nominal as per the Pod Health Checklist (see Table 1).
8. [ ] From the GUI on the base station, retract the brakes for pod mounting. This feature is only available during the IDLE state during pod mounting.
9. [ ] Lift the pod and position it behind the first I-beam segment of the track.
10. [ ] Lower the pod until the front vertical suspension modules are level with the top face of the top I-beam flange.
11. [ ] Move the pod (via the engine hoist or pod lifting mechanism) forward until the front vertical suspension modules are in contact with the track.
12. [ ] Loosen the lifting straps to ensure enough slack to push the pod onto the track. Position 8x people around the pod to stabilise the pod.
13. [ ] Slide the pod onto the track with four people on each side, using the dolly and/or engine hoist.
14. [ ] From the GUI on the base station, clamp (activate) the brakes. This feature is only available during the IDLE state during pod mounting.
15. [ ] Ensure the pod is secured in place. This should be done by the friction brakes.
16. [ ] Begin pod run procedure through the software system by calibrating the systems, transitioning to states: CALIBRATING → READY
17. [ ] Commence the pod run via “RUN” button on the GUI.
18. [ ] After the run, power down the pod via the GUI - High Power should automatically be shut off after the run.
19. [ ] Using electrical insulating gloves, open the shell lid to access inside.
20. [ ] Disconnect High Power via the manual switch on the pod.
21. [ ] Slide the pod off the track onto the dolly and/or engine hoist with four people on each side, using the dolly and/or engine hoist. If using the engine hoist, attach the straps prior to sliding the pod off the track
22. [ ] Depressurise the braking system via the schrader valve.
23. [ ] Remove the pod from the test track location. See Pod Transport procedure.
`;

export function Checklist() {
  const [checkedItems, setCheckedItems] = useState<Record<number, boolean>>({});

  // Handle checkbox toggle
  const handleCheckboxChange = (index: number, event?: React.MouseEvent | React.ChangeEvent) => {
    if (event) event.stopPropagation();

    setCheckedItems((prev) => ({
      ...prev,
      [index]: !prev[index], // toggle checked state of task items
    }));
  };

  return (
    <div className="h-full w-full overflow-y-auto scrollbar scrollbar-thumb-gray-700 scrollbar-track-transparent">
      <Markdown 
        remarkPlugins={[remarkGfm]} 
        components={{
          li: ({ node, className, children, ...props }) => {
            const isTaskListItem = className?.includes("task-list-item");
            let checkbox = null;
            let content = children;
            let index = node?.position?.start?.line ? node.position.start.line - 2 : 0; // numbering tasks (the - 2 is because the first two lines are not tasks)

            if (isTaskListItem) {
              React.Children.forEach(children, (child, idx) => {
                if (idx === 0 && React.isValidElement(child) && child.type === "input") {
                  checkbox = (
                    <input 
                      type="checkbox"
                      checked={checkedItems[index] || false}
                      onChange={(e) => handleCheckboxChange(index, e)}
                      onClick={(e) => e.stopPropagation()} // prevents both checkbox and li onClick from firing
                      className={`appearance-none w-4 h-4 border-2 border-white rounded-sm cursor-pointer ml-auto p-2 flex items-center justify-center
                        ${checkedItems[index] ? `bg-transparent checked:before:content-['✓'] ` : ''}`}
                    />
                  );
                }
              });

              content = React.Children.toArray(children).slice(1);
              const isChecked = checkedItems[index];

              return (
                <li 
                  className="flex items-center justify-between bg-[#222222] m-1 ml-10 mr-10 p-2 rounded-lg cursor-pointer" 
                  {...props}
                  onClick={() => handleCheckboxChange(index)} // makes the entire task clickable
                >
                  <span className={`task-number p-2 text-white ${isChecked ? 'line-through' : ''}`}>{index}.</span>
                  <span className={`task-content p-2 text-white flex-1 pl-2 ${isChecked ? 'line-through' : ''}`}>{content}</span>
                  {checkbox}
                </li>
              );
            }

            return <li {...props}>{children}</li>;
          },
          h1: (props) => <h1 className="p-5 text-[40px] text-white" {...props} />,
        }}
      >
        {markdown}
      </Markdown>
    </div>
  );
}
