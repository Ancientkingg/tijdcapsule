<!-- App.svelte -->
<script>
  import { Router, Link, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Capsule from "./routes/Capsule.svelte";

  export let url = "";
</script>

<Router {url}>
  <div>
    <Route path="/capsule/:id" component={Capsule} />
    <Route path="/"><Home /></Route>
  </div>
</Router>

<style>
  div {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }
</style>