<!-- App.svelte -->
<script>
  import { Router, Link, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Capsule from "./routes/Capsule.svelte";

  export let url = "";
</script>

<Router {url}>
  <nav>
    <Link to="/">Home</Link>
    <Link to="/capsule/1">Capsule 1</Link>
  </nav>
  <div>
    <Route path="/capsule/:id" component={Capsule} />
    <Route path="/"><Home /></Route>
  </div>
</Router>