<script lang="ts">
    import { onDestroy } from "svelte";
    import * as d3 from "d3";

    export let files;

    $: if (files) {
        console.log("build", files);
        buildPlot();
    }

    let svg;

    // Set chart dimensions
    const width = 1800;
    const height = 1000;

    function buildPlot() {
        // const color = d3.scaleOrdinal(d3.schemeCategory10);
        const root = d3.hierarchy(buildHierarchy(files.slice(0, 1000)));

        const links = root.links();
        const nodes = root.descendants();

        // const { nodes, links } = generateNodesAndLinks(files.slice(0, 1000));

        console.log(nodes, links);

        // Initialize the simulation with nodes and link forces.
        // const simulation = d3.forceSimulation(nodes)
        //     .force("link", d3.forceLink(links).id(d => d.id).distance(50).strength(0.5))
        //     .force("charge", d3.forceManyBody().strength(-100))
        //     .force("x", d3.forceX())
        //     .force("y", d3.forceY());

        // Initialize force simulation
        const simulation = d3
            .forceSimulation(nodes)
            .force(
                "link",
                d3.forceLink(links).id((d) => d.id),
            )
            .force("charge", d3.forceManyBody())
            .force("center", d3.forceCenter(width / 2, height / 2))
            .on("tick", ticked);

        // // Create SVG element
        svg = d3
            .select("#graph")
            .append("svg")
            .attr("width", width)
            .attr("height", height)
            .attr("viewBox", [0, 0, width, height])
            .attr("style", "max-width: 100%; height: auto;");

        // svg = d3
        //     .select("#graph")
        //     .append("svg")
        //     .attr("width", width)
        //     .attr("height", height)
        //     .attr("viewBox", [0, 0, width, height])
        //     // .attr("style", "max-width: 100%; height: 100%;");

        // // Create the SVG element and set up zoom and pan.
        // svg = d3.select("graph")
        //     .attr("width", width)
        //     .attr("height", height)
        //     .attr("viewBox", [-width / 2, -height / 2, width, height])
        //     .attr("style", "max-width: 100%; height: auto;")
        //     .call(d3.zoom().on("zoom", (event) => {
        //         svg.attr("transform", event.transform);
        //     }))
        //     .append("g");

        // Append links.
        const link = svg
            .append("g")
            .attr("stroke", "#999")
            .attr("stroke-opacity", 0.6)
            .selectAll("line")
            .data(links)
            .join("line");

        // Append nodes.
        const node = svg
            .append("g")
            .attr("fill", "#fff")
            .attr("stroke", "#000")
            .attr("stroke-width", 1.5)
            .selectAll("circle")
            .data(nodes)
            .join("circle")
            .attr("fill", (d) => (d.children ? null : "#000"))
            .attr("stroke", (d) => (d.children ? null : "#fff"))
            .attr("r", 3.5)
            .call(drag(simulation));

            // .attr("fill", d => d.children ? "#555" : "#000")
            // .attr("r", 5)

        // // Create SVG element
        // svg = d3
        //     .select("#graph")
        //     .append("svg")
        //     .attr("width", width)
        //     .attr("height", height)
        //     .attr("viewBox", [0, 0, width, height])
        //     .attr("style", "max-width: 100%; height: auto;");

        // // Add links and nodes
        // const link = svg
        //     .append("g")
        //     .attr("stroke", "#999")
        //     .attr("stroke-opacity", 0.6)
        //     .selectAll("line")
        //     .data(links)
        //     .join("line")
        //     .attr("stroke-width", (d) => Math.sqrt(d.value));

        // const node = svg
        //     .append("g")
        //     .attr("stroke", "#fff")
        //     .attr("stroke-width", 1.5)
        //     .selectAll("circle")
        //     .data(nodes)
        //     .join("circle")
        //     .attr("r", 4)
        //     .attr("fill", (d) => color(d.group))
        //     .call(drag(simulation));

        node.append("title").text((d) => d.id);

        // Tick function to update positions
        function ticked() {
            link.attr("x1", (d) => d.source.x)
                .attr("y1", (d) => d.source.y)
                .attr("x2", (d) => d.target.x)
                .attr("y2", (d) => d.target.y);

            node.attr("cx", (d) => d.x).attr("cy", (d) => d.y);
        }

        // Drag functions
        function drag(simulation) {
            function dragstarted(event) {
                if (!event.active) simulation.alphaTarget(0.3).restart();
                event.subject.fx = event.subject.x;
                event.subject.fy = event.subject.y;
            }

            function dragged(event) {
                event.subject.fx = event.x;
                event.subject.fy = event.y;
            }

            function dragended(event) {
                if (!event.active) simulation.alphaTarget(0);
                event.subject.fx = null;
                event.subject.fy = null;
            }

            return d3
                .drag()
                .on("start", dragstarted)
                .on("drag", dragged)
                .on("end", dragended);
        }

        onDestroy(() => {
            // Cleanup the SVG and stop the simulation on component unmount
            simulation.stop();
            svg.remove();
        });
    }

    function generateNodesAndLinks(paths) {
        const nodes = new Map();
        const links = [];

        // Step 1: Create a node for each unique directory and file
        paths.forEach((path) => {
            const parts = path.split("/");

            // Build up the path part by part
            parts.reduce((parentPath, part) => {
                const currentPath = parentPath ? `${parentPath}/${part}` : part;

                // Add the current path as a node if it doesn’t already exist
                if (!nodes.has(currentPath)) {
                    nodes.set(currentPath, {
                        id: currentPath,
                        group: parts.length,
                    });
                }

                // If there’s a parent path, create a link to the current path
                if (parentPath) {
                    links.push({ source: parentPath, target: currentPath });
                }

                return currentPath; // The current path becomes the new parentPath for the next iteration
            }, "");
        });

        // Step 2: Convert nodes from Map to array format expected by D3
        const nodesArray = Array.from(nodes.values());

        return { nodes: nodesArray, links };
    }

    function buildHierarchy(paths) {
        const root = { name: "root", children: [] };

        paths.forEach((path) => {
            const parts = path.split("/");
            let current = root;

            parts.forEach((part) => {
                // Find or create the child node
                let child = current.children.find(
                    (child) => child.name === part,
                );
                if (!child) {
                    child = { name: part, children: [] };
                    current.children.push(child);
                }
                // Move to the child for the next iteration
                current = child;
            });
        });

        return root;
    }
</script>

<div class="fixed top-0 left-0 w-full grid place-items-center -z-10">
    <div id="graph"></div>
</div>

<!-- 
<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import * as d3 from "d3";

    export let files;

    $: if (files) {
        console.log("build", files);
        buildPlot();
    }

    // Set chart dimensions
    const width = 1600;
    const height = 980;

    let svg;
    let simulation;

    const buildPlot = () => {
        // const root = d3.hierarchy(files);
        const { links, nodes } = generateNodesAndLinks(files.slice(0, 1000));
        console.log(links, nodes)
        // const links = root.links();
        // const nodes = root.descendants();

        // Initialize the simulation with nodes and link forces.
        simulation = d3.forceSimulation(nodes)
            .force("link", d3.forceLink(links).id(d => d.id).distance(50).strength(0.5))
            .force("charge", d3.forceManyBody().strength(-100))
            .force("x", d3.forceX())
            .force("y", d3.forceY());

        // Create the SVG element and set up zoom and pan.
        svg = d3.select("graph")
            .attr("width", width)
            .attr("height", height)
            .attr("viewBox", [-width / 2, -height / 2, width, height])
            .attr("style", "max-width: 100%; height: auto;")
            .call(d3.zoom().on("zoom", (event) => {
                svg.attr("transform", event.transform);
            }))
            .append("g");

        // Append links.
        const link = svg.append("g")
            .attr("stroke", "#999")
            .attr("stroke-opacity", 0.6)
            .selectAll("line")
            .data(links)
            .join("line");

        // Append nodes.
        const node = svg.append("g")
            .attr("stroke", "#000")
            .attr("stroke-width", 1.5)
            .selectAll("circle")
            .data(nodes)
            .join("circle")
            .attr("fill", d => d.children ? "#555" : "#000")
            .attr("r", 5)
            .call(drag(simulation));

        // Add tooltips with node names.
        node.append("title").text((d) => d.id);

        // Update positions on each tick of the simulation.
        simulation.on("tick", () => {
            link
                .attr("x1", d => d.source.x)
                .attr("y1", d => d.source.y)
                .attr("x2", d => d.target.x)
                .attr("y2", d => d.target.y);

            node
                .attr("cx", d => d.x)
                .attr("cy", d => d.y);
        });
    };

    // Cleanup the simulation on component destroy to prevent memory leaks.
    onDestroy(() => {
        if (simulation) simulation.stop();
    });

    // Function to handle dragging nodes.
    function drag(simulation) {
        function dragstarted(event, d) {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
        }

        function dragged(event, d) {
            d.fx = event.x;
            d.fy = event.y;
        }

        function dragended(event, d) {
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
        }

        return d3.drag()
            .on("start", dragstarted)
            .on("drag", dragged)
            .on("end", dragended);
    }

    function generateNodesAndLinks(paths) {
        const nodes = new Map();
        const links = [];

        // Step 1: Create a node for each unique directory and file
        paths.forEach((path) => {
            const parts = path.split("/");

            // Build up the path part by part
            parts.reduce((parentPath, part) => {
                const currentPath = parentPath ? `${parentPath}/${part}` : part;

                // Add the current path as a node if it doesn’t already exist
                if (!nodes.has(currentPath)) {
                    nodes.set(currentPath, {
                        id: currentPath,
                        group: parts.length,
                    });
                }

                // If there’s a parent path, create a link to the current path
                if (parentPath) {
                    links.push({ source: parentPath, target: currentPath });
                }

                return currentPath; // The current path becomes the new parentPath for the next iteration
            }, "");
        });

        // Step 2: Convert nodes from Map to array format expected by D3
        const nodesArray = Array.from(nodes.values());

        return { nodes: nodesArray, links };
    }
</script>

<div class="fixed top-0 left-0 w-full grid place-items-center">
    <div id="graph"></div>
</div> -->
