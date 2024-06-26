/*
  Warnings:

  - You are about to drop the column `tvl` on the `EtherFi` table. All the data in the column will be lost.
  - Added the required column `eth` to the `EtherFi` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "EtherFi" DROP COLUMN "tvl",
ADD COLUMN     "eth" TEXT NOT NULL;
