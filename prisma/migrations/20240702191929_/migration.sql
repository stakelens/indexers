/*
  Warnings:

  - You are about to drop the column `log_index` on the `Swell` table. All the data in the column will be lost.
  - You are about to drop the column `rate` on the `Swell` table. All the data in the column will be lost.
  - You are about to drop the column `total_supply` on the `Swell` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[block_number]` on the table `Swell` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `eth` to the `Swell` table without a default value. This is not possible if the table is not empty.

*/
-- DropIndex
DROP INDEX "Swell_block_number_log_index_key";

-- AlterTable
ALTER TABLE "Swell" DROP COLUMN "log_index",
DROP COLUMN "rate",
DROP COLUMN "total_supply",
ADD COLUMN     "eth" TEXT NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "Swell_block_number_key" ON "Swell"("block_number");
