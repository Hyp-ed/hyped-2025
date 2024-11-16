import { Inject, Injectable, OnApplicationBootstrap } from '@nestjs/common';
import * as schema from './schema';
import { NodePgDatabase } from 'drizzle-orm/node-postgres';
import { TAG } from './Postgres.module';

@Injectable()
export class PostgresService implements OnApplicationBootstrap {
  constructor(
    @Inject(TAG) private drizzle: NodePgDatabase<typeof schema>,
  ) {}
  async getData() {
    const measurements = await this.drizzle.query.measurements.findMany();
    return {
      measurements,
    };
  }
  async onApplicationBootstrap() {
    const data = await this.getData();
    console.log(data);
  }
  
}
