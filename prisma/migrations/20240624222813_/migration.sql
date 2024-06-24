/*
  Warnings:

  - The primary key for the `EtherFi` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `RocketPoolTVL` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - Added the required column `block_timestamp` to the `EtherFi` table without a default value. This is not possible if the table is not empty.
  - Added the required column `log_index` to the `EtherFi` table without a default value. This is not possible if the table is not empty.
  - Added the required column `block_timestamp` to the `RocketPoolTVL` table without a default value. This is not possible if the table is not empty.
  - Added the required column `log_index` to the `RocketPoolTVL` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_EtherFi" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "tvl" BIGINT NOT NULL
);
INSERT INTO "new_EtherFi" ("block_number", "tvl") SELECT "block_number", "tvl" FROM "EtherFi";
DROP TABLE "EtherFi";
ALTER TABLE "new_EtherFi" RENAME TO "EtherFi";
CREATE UNIQUE INDEX "EtherFi_block_number_log_index_key" ON "EtherFi"("block_number", "log_index");
CREATE TABLE "new_RocketPoolTVL" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "eth" BIGINT NOT NULL,
    "rpl" BIGINT NOT NULL
);
INSERT INTO "new_RocketPoolTVL" ("block_number", "eth", "rpl") SELECT "block_number", "eth", "rpl" FROM "RocketPoolTVL";
DROP TABLE "RocketPoolTVL";
ALTER TABLE "new_RocketPoolTVL" RENAME TO "RocketPoolTVL";
CREATE UNIQUE INDEX "RocketPoolTVL_block_number_log_index_key" ON "RocketPoolTVL"("block_number", "log_index");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
