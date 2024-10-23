import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export const cn = (...inputs: ClassValue[]) => {
  return twMerge(clsx(inputs));
};

/**
 * Gets the full MQTT topic name for a given partial topic and podId
 * @param topic The partial topic name (will be prefixed with ```hyped/${podId}/```)
 * @param podId The podId to use in the topic
 * @returns
 */
export const getTopic = (topic: string, podId: string) =>
  `hyped/${podId}/${topic}`;
