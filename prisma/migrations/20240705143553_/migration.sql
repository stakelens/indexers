-- CreateTable
CREATE TABLE "Stader" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "eth" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Stader_block_number_key" ON "Stader"("block_number");
