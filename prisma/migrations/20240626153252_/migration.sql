/*
  Warnings:

  - Added the required column `block_timestamp` to the `EtherFi` table without a default value. This is not possible if the table is not empty.
  - Added the required column `block_timestamp` to the `RocketPool` table without a default value. This is not possible if the table is not empty.
  - Added the required column `block_timestamp` to the `StakeWise` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "EtherFi" ADD COLUMN     "block_timestamp" BIGINT NOT NULL;

-- AlterTable
ALTER TABLE "RocketPool" ADD COLUMN     "block_timestamp" BIGINT NOT NULL;

-- AlterTable
ALTER TABLE "StakeWise" ADD COLUMN     "block_timestamp" BIGINT NOT NULL;
