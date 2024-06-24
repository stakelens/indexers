/*
  Warnings:

  - You are about to drop the column `assets` on the `StakeWise` table. All the data in the column will be lost.
  - Added the required column `eth` to the `StakeWise` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_StakeWise" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "vault" TEXT NOT NULL,
    "eth" BIGINT NOT NULL
);
INSERT INTO "new_StakeWise" ("block_number", "log_index", "vault") SELECT "block_number", "log_index", "vault" FROM "StakeWise";
DROP TABLE "StakeWise";
ALTER TABLE "new_StakeWise" RENAME TO "StakeWise";
CREATE UNIQUE INDEX "StakeWise_block_number_log_index_key" ON "StakeWise"("block_number", "log_index");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
