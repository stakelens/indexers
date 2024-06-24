/*
  Warnings:

  - You are about to drop the `ETHVault` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "ETHVault";
PRAGMA foreign_keys=on;

-- CreateTable
CREATE TABLE "StakeWise" (
    "block_number" BIGINT NOT NULL,
    "log_index" BIGINT NOT NULL,
    "vault" TEXT NOT NULL,
    "assets" BIGINT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "StakeWise_block_number_log_index_key" ON "StakeWise"("block_number", "log_index");
