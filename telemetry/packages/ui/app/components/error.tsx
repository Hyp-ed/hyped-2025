import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogTitle,
} from '@/components/ui/alert-dialog';
import { useErrors } from '@/context/errors';
import { useCurrentPod } from '@/context/pods';
import { CONTROLS, sendControlMessage } from '@/lib/controls';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { useState } from 'react';
import { Button } from './ui/button';
import { DialogHeader } from './ui/dialog';

/**
 * Dialog to display error messages from the error context.
 */
export const Error = () => {
	const { errors } = useErrors();
	const [index, setIndex] = useState(0);

	const { currentPod } = useCurrentPod();

	const open = errors.length > 0;

	return (
		<AlertDialog open={open}>
			<AlertDialogContent>
				{/* Error message */}
				{errors[index] && (
					<>
						<DialogHeader>
							<AlertDialogTitle>{errors[index].title}</AlertDialogTitle>
						</DialogHeader>
						<AlertDialogDescription>
							{errors[index].message}
						</AlertDialogDescription>
					</>
				)}

				{errors[index] && (
					<AlertDialogFooter>
						<div className="flex w-full items-center justify-between text-sm">
							{/* Arrows for navigating between errors */}
							<div className="flex gap-2 items-center">
								<button
									onClick={() => setIndex((i) => i - 1)}
									disabled={index === 0}
									className="disabled:opacity-50"
								>
									<ChevronLeft />
								</button>
								<p>
									{index + 1} / {errors.length}
								</p>
								<button
									onClick={() => setIndex((i) => i + 1)}
									disabled={index === errors.length - 1}
									className="disabled:opacity-50"
								>
									<ChevronRight />
								</button>
							</div>
							<div className="flex gap-2">
								{/* Emergency Stop Button */}
								<Button
									className="bg-black border-2 border-red-600 hover:border-red-700 hover:bg-red-700"
									onClick={() =>
										void sendControlMessage(
											errors[index].podId || currentPod,
											CONTROLS.STOP,
										)
									}
								>
									Emergency Stop ({errors[index].podId || currentPod})
								</Button>
								{/* Acknowledge button */}
								<AlertDialogAction
									onClick={() => {
										errors[index].acknowledge();
										setIndex((i) => (i > 0 ? i - 1 : 0));
									}}
								>
									Acknowledge
								</AlertDialogAction>
							</div>
						</div>
					</AlertDialogFooter>
				)}
			</AlertDialogContent>
		</AlertDialog>
	);
};
