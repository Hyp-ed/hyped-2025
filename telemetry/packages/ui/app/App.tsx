import { useState } from 'react';
import { VIEWS, VIEW_KEYS, VIEW_OPTIONS, ViewOption } from './views';
import { Sidebar } from './components/sidebar';
import { cn } from './lib/utils';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from '@/components/ui/resizable';

/**
 * The default view to display when the app is opened.
 */
const DEFAULT_VIEW: ViewOption = VIEW_OPTIONS.OPEN_MCT;

export const App = () => {
  const [currentView, setCurrentView] = useState<ViewOption>(DEFAULT_VIEW);

  return (
    <ResizablePanelGroup
      direction="horizontal"
      className="w-full bg-hyped-background"
    >
      <ResizablePanel defaultSize={85} minSize={50}>
        {VIEW_KEYS.map((key) => (
          <div
            key={key}
            className={cn(
              'h-[100vh] w-full col-span-7 p-1',
              currentView !== key && 'hidden',
            )}
          >
            {VIEWS[key].component}
          </div>
        ))}
      </ResizablePanel>
      <ResizableHandle withHandle className="bg-openmct-light-gray" />
      <ResizablePanel defaultSize={15} minSize={10}>
        <Sidebar currentView={currentView} setCurrentView={setCurrentView} />
      </ResizablePanel>
    </ResizablePanelGroup>
  );
};
