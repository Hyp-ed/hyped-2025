import * as env from 'env-var';

export const ENV = env.get('ENV').default('development').asString();

export const INFLUX_URL = env.get('INFLUX_URL').required().asUrlString();
export const INFLUX_TOKEN = env.get('INFLUX_TOKEN').required().asString();
export const INFLUX_ORG = env.get('INFLUX_ORG').required().asString();
export const INFLUX_TELEMETRY_BUCKET = env
  .get('INFLUX_TELEMETRY_BUCKET')
  .required()
  .asString();
export const INFLUX_FAULTS_BUCKET = env
  .get('INFLUX_FAULTS_BUCKET')
  .required()
  .asString();

export const MQTT_BROKER_HOST = env
  .get('MQTT_BROKER_HOST')
  .required()
  .asString();
