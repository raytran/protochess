<script>
    //{x, y}
    export let pos;
    export let gameDimensions;
    export let tileDimensions;
    export let flipped = false;

</script>

<div
     style="width:{tileDimensions.width + '%'};
             height:{tileDimensions.height + '%'};
             position: absolute;
             left: {(!flipped ? pos.x : gameDimensions.width - pos.x - 1) * tileDimensions.width + '%'};
             bottom: {(!flipped ? pos.y : gameDimensions.height - pos.y - 1)  * tileDimensions.height + '%'};
             background-color: {'rgba(200, 0, 200, 0.3)'}">
</div>
