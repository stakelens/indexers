-- CreateTable
CREATE TABLE "Lido" (
    "block_number" BIGINT NOT NULL,
    "block_timestamp" BIGINT NOT NULL,
    "eth" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Lido_block_number_key" ON "Lido"("block_number");
