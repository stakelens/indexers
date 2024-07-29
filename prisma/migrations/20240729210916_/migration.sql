/*
  Warnings:

  - The primary key for the `CurrencyPrice` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `date` on the `CurrencyPrice` table. All the data in the column will be lost.
  - Added the required column `block_timestamp` to the `CurrencyPrice` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "CurrencyPrice" DROP CONSTRAINT "CurrencyPrice_pkey",
DROP COLUMN "date",
ADD COLUMN     "block_timestamp" BIGINT NOT NULL,
ADD CONSTRAINT "CurrencyPrice_pkey" PRIMARY KEY ("name", "block_timestamp");
