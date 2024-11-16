import { pgEnum, pgTable as table } from "drizzle-orm/pg-core";
import * as t from "drizzle-orm/pg-core";

export const measurements = table('measurements', {
  id: t.integer(),
  podId: t.integer().notNull(),
  measurementKey: t.varchar({ length: 256 }).notNull(),
  value: t.varchar(),
})
