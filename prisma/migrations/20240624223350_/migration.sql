/*
  Warnings:

  - You are about to drop the column `block_timestamp` on the `EtherFi` table. All the data in the column will be lost.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_EtherFi" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "tvl" BIGINT NOT NULL
);
INSERT INTO "new_EtherFi" ("block_number", "log_index", "tvl") SELECT "block_number", "log_index", "tvl" FROM "EtherFi";
DROP TABLE "EtherFi";
ALTER TABLE "new_EtherFi" RENAME TO "EtherFi";
CREATE UNIQUE INDEX "EtherFi_block_number_log_index_key" ON "EtherFi"("block_number", "log_index");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
