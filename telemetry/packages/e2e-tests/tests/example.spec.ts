import { expect, test } from '@playwright/test';
import { client, validateMqttMessage } from '../lib/mqtt';

test('has title', async ({ page }) => {
	await page.goto('http://localhost:5173');

	await expect(page).toHaveTitle('HYPED24 | Telemetry');
});

test('example mqtt test', async () => {
	await validateMqttMessage(
		// Pass in the function which will trigger the MQTT message to be sent.
		// For example, this could be pushing a button on the GUI.
		() => {
			client.publish('hello', 'world');
		},
		// The validation function. Here you can validate that the topic and message body received is as expected.
		(topic, message) => {
			expect(topic).toBe('hello');
			expect(message.toString()).toBe('world');
		},
	);
});
