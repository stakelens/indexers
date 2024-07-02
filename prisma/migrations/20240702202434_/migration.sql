-- CreateTable
CREATE TABLE "Renzo" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "eth" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Renzo_block_number_key" ON "Renzo"("block_number");
