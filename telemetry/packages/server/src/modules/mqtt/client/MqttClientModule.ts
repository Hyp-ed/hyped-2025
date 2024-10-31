import { Module } from '@nestjs/common';
import { MqttModule } from 'nest-mqtt';
import { MQTT_BROKER_HOST } from 'src/modules/core/config';

const mqttClient = MqttModule.forRoot({
  host: MQTT_BROKER_HOST,
});

@Module({
  imports: [mqttClient],
  exports: [mqttClient],
})
export class MqttClientModule {}
