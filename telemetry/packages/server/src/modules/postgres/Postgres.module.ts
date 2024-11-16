import { Module } from '@nestjs/common';
import * as schema from './schema';
import { DrizzlePGModule } from '@knaadh/nestjs-drizzle-pg';
import { POSTGRES_CONNECTION_STRING } from '../core/config';
import { PostgresService } from './Postgres.service';

export const TAG = 'DB';

const drizzleClient = DrizzlePGModule.register({
    tag: TAG,
    pg: {
      connection: 'client',
      config: {
        connectionString: POSTGRES_CONNECTION_STRING,
      },
    },
    config: { schema: { ...schema }, casing: 'snake_case'},
  })

@Module({
  imports: [drizzleClient],
  exports: [drizzleClient],
  providers: [PostgresService]
})
export class PostgresModule {}
