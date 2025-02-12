import React from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

const markdown = `
# Checklist
---
- [ ] Ensure all non-essential personnel are cleared from the area to the designated observation zone. 
- [x] Transport the pod to the track using the Pod Transport Procedure (see SSOW).
- [ ] Position the pod on the ground/cart behind the first I-beam segment of the track (shell off).

| | Safe System of Work Instructions |
| ----------- | ----------- |
| 1 | Ensure all non-essential personnel are cleared from the area to the designated observation zone. |
| 2 | Transport the pod to the track using the Pod Transport Procedure (see SSOW). |
| 3 | Position the pod on the ground/cart behind the first I-beam segment of the track (shell off). |
| 4 | Pressurise the pneumatic tank through the schrader valve. Use the air compressor to take note of the pressure. |
`;

export function Checklist() {
  return (
    <Markdown 
      rehypePlugins={[remarkGfm]} 
      components={{
        h1(props) {
          const {node, ...rest} = props
          return <h1 className="p-5 text-center text-[40px] color-white" {...rest} />
        },
        input(props) {
          const {node, ...rest} = props
          return <input className="appearance-none bg-transparent w-4 h-4 border-2 border-white rounded-sm ml-20 cursor-pointer" {...rest} />
        },
        hr(props) {
          const {node, ...rest} = props
          return <hr className="border-2 mb-10 mr-20 ml-20" {...rest} />
        },   
        table(props) {
          const {node, ...rest} = props
          return <table className="" {...rest} />
        },   
        th(props) {
          const {node, ...rest} = props
          return <td className="border-2 p-3 border-white" {...rest} />
        }, 
        td(props) {
          const {node, ...rest} = props
          return <td className="border-2 p-3 border-white" {...rest} />
        }, 
      }}>{markdown}</Markdown>
  );
};

