<script>
    let q_and_a = [
        {
            question: "What does \"create a new group\" mean in the piece editor?",
            answer: "There are two kinds of pieces in chess -- \"jumping\" and \"sliding\" pieces. A jumping piece is like the knight; it can \"jump\" over other pieces. A \"sliding piece\" like the queen requires the spaces in front of it to be clear to move. We can think of a \"slide group\" as a group of tiles in which the order of the square represents the order that tiles must be free for the piece to move/attack. The \"new group\" button in the editor starts a new, independent slide group. This is important since the \"slide squares\" don't necessarily have to be connected."
        },
        {
            question: "Why can't I save changes to my board?",
            answer: "The only requirement to a \"valid\" board is for both sides to have a king. You can have pieces start on grey squares, have pieces that don't have any movement associated with them, or anything else as long as both sides have a king."
        },
        {
            question: "How does castling on my custom boards work?",
            answer: "At the moment, castling is defined as moving the king two squares either kingside or queenside, and then moving the rook onto the respective tile. Rooks must be placed on either the first or last file. This might change in the future. This is different from chess960 where castling always results in the same king position."
        },
        {
            question: "Can I save changes to my board between game sessions?",
            answer: "Not yet. Maybe in the future."
        },
        {
            question: "Do you play chess?",
            answer: "Sometimes. I'm not that good though."
        },
    ];
</script>
<div class="container">
    <div class="box">
        <h1 class="title">FAQ</h1>
        <h2 class="subtitle">Frequently Asked Questions</h2>
        {#each q_and_a as qa}
            <div class="container">
                <b class="is-size-4">{qa.question}</b>
                <p>{qa.answer}</p>
            </div>
        {/each}
    </div>
</div>
