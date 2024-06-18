<script>
    import { ModeWatcher } from "mode-watcher";
    import "../app.css";

    // @ts-ignore
    window.enableCORSFetch(true);
</script>

<ModeWatcher />
<slot></slot>