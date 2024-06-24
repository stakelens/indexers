/*
  Warnings:

  - You are about to drop the `Test` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "Test";
PRAGMA foreign_keys=on;

-- CreateTable
CREATE TABLE "RocketPoolTVL" (
    "block_number" BIGINT NOT NULL PRIMARY KEY,
    "eth" BIGINT NOT NULL,
    "rpl" BIGINT NOT NULL
);
