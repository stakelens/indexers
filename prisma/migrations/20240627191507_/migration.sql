/*
  Warnings:

  - You are about to drop the column `log_index` on the `RocketPool` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[block_number]` on the table `RocketPool` will be added. If there are existing duplicate values, this will fail.

*/
-- DropIndex
DROP INDEX "RocketPool_block_number_log_index_key";

-- AlterTable
ALTER TABLE "RocketPool" DROP COLUMN "log_index";

-- CreateIndex
CREATE UNIQUE INDEX "RocketPool_block_number_key" ON "RocketPool"("block_number");
