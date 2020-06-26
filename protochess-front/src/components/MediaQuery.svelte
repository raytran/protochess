<style>
    #match {
        display: none;
    }
    #default {
        display: block;
    }
    @media (min-width: 1400px){
        #match {
            display: block;
        }
        #default{
            display: none;
        }
    }
</style>
<div id="match">
    <slot name="match"/>
</div>
<div id="default">
    <slot name="fallback"/>
</div>
<!-- always display -->
<slot/>