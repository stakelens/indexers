/*
  Warnings:

  - You are about to drop the column `block_timestamp` on the `RocketPoolTVL` table. All the data in the column will be lost.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_RocketPoolTVL" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "eth" BIGINT NOT NULL,
    "rpl" BIGINT NOT NULL
);
INSERT INTO "new_RocketPoolTVL" ("block_number", "eth", "log_index", "rpl") SELECT "block_number", "eth", "log_index", "rpl" FROM "RocketPoolTVL";
DROP TABLE "RocketPoolTVL";
ALTER TABLE "new_RocketPoolTVL" RENAME TO "RocketPoolTVL";
CREATE UNIQUE INDEX "RocketPoolTVL_block_number_log_index_key" ON "RocketPoolTVL"("block_number", "log_index");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
