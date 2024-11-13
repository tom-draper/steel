<script lang="ts">
    import { onDestroy } from "svelte";
    import * as d3 from "d3";

    export let files;

    $: if (files) {
        console.log("build", files);
        buildPlot();
    }
    //   export let data = { nodes: [], links: [] };
    const data = {
        nodes: [
            { id: "1", group: 1 },
            { id: "2", group: 2 },
            // additional nodes...
        ],
        links: [
            { source: "1", target: "2", value: 1 },
            // additional links...
        ],
    };
    let svg;

    // Set chart dimensions
    const width = 1600;
    const height = 980;

    function buildPlot() {
        const color = d3.scaleOrdinal(d3.schemeCategory10);

        // const links = data.links.map((d) => ({ ...d }));
        // const nodes = data.nodes.map((d) => ({ ...d }));
        console.log(files);
        const { nodes, links } = generateNodesAndLinks(files.slice(0, 1000));

        console.log(nodes, links);

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

        // Create SVG element
        svg = d3
            .select("#graph")
            .append("svg")
            .attr("width", width)
            .attr("height", height)
            .attr("viewBox", [0, 0, width, height])
            .attr("style", "max-width: 100%; height: auto;");

        // Add links and nodes
        const link = svg
            .append("g")
            .attr("stroke", "#999")
            .attr("stroke-opacity", 0.6)
            .selectAll("line")
            .data(links)
            .join("line")
            .attr("stroke-width", (d) => Math.sqrt(d.value));

        const node = svg
            .append("g")
            .attr("stroke", "#fff")
            .attr("stroke-width", 1.5)
            .selectAll("circle")
            .data(nodes)
            .join("circle")
            .attr("r", 4)
            .attr("fill", (d) => color(d.group))
            .call(drag(simulation));

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
</script>

<div class="fixed top-0 left-0 w-full grid place-items-center">
    <div id="graph"></div>
</div>
