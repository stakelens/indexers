/*
  Warnings:

  - The primary key for the `CurrencyPrice` table will be changed. If it partially fails, the table could be left without primary key constraint.

*/
-- DropIndex
DROP INDEX "CurrencyPrice_name_date_key";

-- AlterTable
ALTER TABLE "CurrencyPrice" DROP CONSTRAINT "CurrencyPrice_pkey",
ADD CONSTRAINT "CurrencyPrice_pkey" PRIMARY KEY ("name", "date");
